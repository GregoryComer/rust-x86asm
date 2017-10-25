use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fnsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectDisplaced(BP, 9478, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 182, 6, 37], OperandSize::Word)
}

fn fnsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 61369145, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 180, 179, 57, 107, 168, 3], OperandSize::Dword)
}

fn fnsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSAVE, operand1: Some(IndirectScaledDisplaced(RAX, Four, 828577825, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 52, 133, 33, 24, 99, 49], OperandSize::Qword)
}

