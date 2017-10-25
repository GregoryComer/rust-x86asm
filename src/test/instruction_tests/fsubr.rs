use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fsubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(SI, 5315, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 172, 195, 20], OperandSize::Word)
}

fn fsubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledDisplaced(EDX, Two, 1371153930, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 44, 85, 10, 38, 186, 81], OperandSize::Dword)
}

fn fsubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 44, 70], OperandSize::Qword)
}

fn fsubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 238], OperandSize::Word)
}

fn fsubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 238], OperandSize::Dword)
}

fn fsubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 238], OperandSize::Qword)
}

fn fsubr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(DI, 31460, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 173, 228, 122], OperandSize::Word)
}

fn fsubr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 40], OperandSize::Dword)
}

fn fsubr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(RDX, 137039458, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 170, 98, 14, 43, 8], OperandSize::Qword)
}

fn fsubr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 225], OperandSize::Word)
}

fn fsubr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 225], OperandSize::Dword)
}

fn fsubr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 230], OperandSize::Qword)
}

