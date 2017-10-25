use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 243], OperandSize::Word)
}

#[test]
fn cmovc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(BP, 144, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 150, 144, 0], OperandSize::Word)
}

#[test]
fn cmovc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 219], OperandSize::Dword)
}

#[test]
fn cmovc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1043605807, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 52, 125, 47, 41, 52, 62], OperandSize::Dword)
}

#[test]
fn cmovc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 211], OperandSize::Qword)
}

#[test]
fn cmovc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1962929404, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 20, 125, 252, 236, 255, 116], OperandSize::Qword)
}

#[test]
fn cmovc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 229], OperandSize::Word)
}

#[test]
fn cmovc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BX, 44, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 127, 44], OperandSize::Word)
}

#[test]
fn cmovc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 253], OperandSize::Dword)
}

#[test]
fn cmovc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESP)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 38], OperandSize::Dword)
}

#[test]
fn cmovc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 252], OperandSize::Qword)
}

#[test]
fn cmovc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 63], OperandSize::Qword)
}

#[test]
fn cmovc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 252], OperandSize::Qword)
}

#[test]
fn cmovc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 34], OperandSize::Qword)
}

