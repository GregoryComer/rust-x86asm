use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 27], OperandSize::Word)
}

fn fcomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectDisplaced(EDX, 1758786782, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 154, 222, 244, 212, 104], OperandSize::Dword)
}

fn fcomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 25], OperandSize::Qword)
}

fn fcomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 218], OperandSize::Word)
}

fn fcomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 221], OperandSize::Dword)
}

fn fcomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 218], OperandSize::Qword)
}

fn fcomp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectDisplaced(BP, 11107, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 158, 99, 43], OperandSize::Word)
}

fn fcomp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 28, 222], OperandSize::Dword)
}

fn fcomp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 28, 216], OperandSize::Qword)
}

