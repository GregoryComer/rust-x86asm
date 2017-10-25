use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 8, 255], OperandSize::Dword)
}

#[test]
fn vpsignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 2130886616, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 8, 148, 207, 216, 191, 2, 127], OperandSize::Dword)
}

#[test]
fn vpsignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 8, 233], OperandSize::Qword)
}

#[test]
fn vpsignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 8, 52, 210], OperandSize::Qword)
}

#[test]
fn vpsignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 8, 196], OperandSize::Dword)
}

#[test]
fn vpsignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1130024256, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 8, 148, 87, 64, 205, 90, 67], OperandSize::Dword)
}

#[test]
fn vpsignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 8, 233], OperandSize::Qword)
}

#[test]
fn vpsignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 2028871134, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 8, 60, 189, 222, 29, 238, 120], OperandSize::Qword)
}

