use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsf_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 249], OperandSize::Word)
}

#[test]
fn bsf_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 14245, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 168, 165, 55], OperandSize::Word)
}

#[test]
fn bsf_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 244], OperandSize::Dword)
}

#[test]
fn bsf_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 287719118, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 12, 213, 206, 62, 38, 17], OperandSize::Dword)
}

#[test]
fn bsf_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 228], OperandSize::Qword)
}

#[test]
fn bsf_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1100527428, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 12, 69, 68, 183, 152, 65], OperandSize::Qword)
}

#[test]
fn bsf_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 221], OperandSize::Word)
}

#[test]
fn bsf_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBX)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 31], OperandSize::Word)
}

#[test]
fn bsf_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 213], OperandSize::Dword)
}

#[test]
fn bsf_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ECX, 1315138899, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 153, 83, 109, 99, 78], OperandSize::Dword)
}

#[test]
fn bsf_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 254], OperandSize::Qword)
}

#[test]
fn bsf_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1725867853, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 44, 85, 77, 167, 222, 102], OperandSize::Qword)
}

#[test]
fn bsf_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 242], OperandSize::Qword)
}

#[test]
fn bsf_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 40], OperandSize::Qword)
}

