use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 8, 230], OperandSize::Dword)
}

#[test]
fn vpsignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 8, 52, 78], OperandSize::Dword)
}

#[test]
fn vpsignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 8, 234], OperandSize::Qword)
}

#[test]
fn vpsignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1756427055, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 8, 156, 65, 47, 243, 176, 104], OperandSize::Qword)
}

#[test]
fn vpsignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 8, 201], OperandSize::Dword)
}

#[test]
fn vpsignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 2068534613, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 8, 164, 190, 85, 85, 75, 123], OperandSize::Dword)
}

#[test]
fn vpsignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 8, 214], OperandSize::Qword)
}

#[test]
fn vpsignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1563209070, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 8, 140, 150, 110, 173, 44, 93], OperandSize::Qword)
}

