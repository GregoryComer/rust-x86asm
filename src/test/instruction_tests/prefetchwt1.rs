use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn prefetchwt1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHWT1, operand1: Some(IndirectDisplaced(ESI, 635476016, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 150, 48, 152, 224, 37], OperandSize::Dword)
}

fn prefetchwt1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHWT1, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 19], OperandSize::Qword)
}

