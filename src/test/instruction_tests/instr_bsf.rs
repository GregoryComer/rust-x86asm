use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsf_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 215], OperandSize::Word)
}

#[test]
fn bsf_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 4570, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 145, 218, 17], OperandSize::Word)
}

#[test]
fn bsf_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 243], OperandSize::Dword)
}

#[test]
fn bsf_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 153029365, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 156, 142, 245, 10, 31, 9], OperandSize::Dword)
}

#[test]
fn bsf_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 204], OperandSize::Qword)
}

#[test]
fn bsf_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1869712613, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 20, 205, 229, 140, 113, 111], OperandSize::Qword)
}

#[test]
fn bsf_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 244], OperandSize::Word)
}

#[test]
fn bsf_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 242, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 188, 186, 242, 0], OperandSize::Word)
}

#[test]
fn bsf_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 246], OperandSize::Dword)
}

#[test]
fn bsf_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 52, 194], OperandSize::Dword)
}

#[test]
fn bsf_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 253], OperandSize::Qword)
}

#[test]
fn bsf_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RBX, 626226505, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 188, 163, 73, 117, 83, 37], OperandSize::Qword)
}

#[test]
fn bsf_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 236], OperandSize::Qword)
}

#[test]
fn bsf_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSF, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 188, 40], OperandSize::Qword)
}

