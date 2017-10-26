use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 239, 90], OperandSize::Dword)
}

#[test]
fn vextractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1037981238, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 60, 133, 54, 86, 222, 61, 113], OperandSize::Dword)
}

#[test]
fn vextractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 217, 48], OperandSize::Qword)
}

#[test]
fn vextractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectDisplaced(RDI, 1166792373, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 135, 181, 214, 139, 69, 75], OperandSize::Qword)
}

#[test]
fn vextractps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 243, 28], OperandSize::Dword)
}

#[test]
fn vextractps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 18, 33], OperandSize::Dword)
}

#[test]
fn vextractps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 204, 100], OperandSize::Qword)
}

#[test]
fn vextractps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM14)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 121, 23, 49, 55], OperandSize::Qword)
}

