use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 224], OperandSize::Dword)
}

#[test]
fn punpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 226080258, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 140, 144, 2, 182, 121, 13], OperandSize::Dword)
}

#[test]
fn punpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 240], OperandSize::Qword)
}

#[test]
fn punpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 36, 144], OperandSize::Qword)
}

#[test]
fn punpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 195], OperandSize::Dword)
}

#[test]
fn punpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 638521727, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 172, 80, 127, 17, 15, 38], OperandSize::Dword)
}

#[test]
fn punpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 203], OperandSize::Qword)
}

#[test]
fn punpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDI, 1477171382, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 167, 182, 216, 11, 88], OperandSize::Qword)
}

