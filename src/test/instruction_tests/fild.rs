use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fild_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(BX, 27695, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 135, 47, 108], OperandSize::Word)
}

fn fild_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 4, 90], OperandSize::Dword)
}

fn fild_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 165570546, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 4, 205, 242, 103, 222, 9], OperandSize::Qword)
}

fn fild_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 139, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 128, 139, 0], OperandSize::Word)
}

fn fild_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(ESI, 1684927473, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 134, 241, 243, 109, 100], OperandSize::Dword)
}

fn fild_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 6], OperandSize::Qword)
}

fn fild_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 40], OperandSize::Word)
}

fn fild_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 44, 90], OperandSize::Dword)
}

fn fild_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 733465311, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 172, 186, 223, 202, 183, 43], OperandSize::Qword)
}

