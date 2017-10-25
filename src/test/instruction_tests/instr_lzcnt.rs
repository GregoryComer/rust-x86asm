use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 255], OperandSize::Word)
}

#[test]
fn lzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 12487, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 136, 199, 48], OperandSize::Word)
}

#[test]
fn lzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 255], OperandSize::Dword)
}

#[test]
fn lzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1323495522, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 44, 245, 98, 240, 226, 78], OperandSize::Dword)
}

#[test]
fn lzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 217], OperandSize::Qword)
}

#[test]
fn lzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 2142583456, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 20, 221, 160, 58, 181, 127], OperandSize::Qword)
}

#[test]
fn lzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 218], OperandSize::Word)
}

#[test]
fn lzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(DI, 17739, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 173, 75, 69], OperandSize::Word)
}

#[test]
fn lzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 235], OperandSize::Dword)
}

#[test]
fn lzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EBX, 730057188, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 171, 228, 201, 131, 43], OperandSize::Dword)
}

#[test]
fn lzcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 238], OperandSize::Qword)
}

#[test]
fn lzcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 701254399, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 148, 192, 255, 74, 204, 41], OperandSize::Qword)
}

#[test]
fn lzcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 202], OperandSize::Qword)
}

#[test]
fn lzcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1769160399, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 140, 201, 207, 62, 115, 105], OperandSize::Qword)
}

