use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn clwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 52, 159], OperandSize::Dword)
}

fn clwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 707041190, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 180, 95, 166, 151, 36, 42], OperandSize::Qword)
}

