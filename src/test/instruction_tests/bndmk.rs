use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bndmk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1033003871, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 12, 93, 95, 99, 146, 61], OperandSize::Dword)
}

fn bndmk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND2)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 23], OperandSize::Qword)
}

