use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 8, 207], OperandSize::Dword)
}

#[test]
fn vpsignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 1930926134, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 8, 176, 54, 152, 23, 115], OperandSize::Dword)
}

#[test]
fn vpsignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 8, 227], OperandSize::Qword)
}

#[test]
fn vpsignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 8, 44, 190], OperandSize::Qword)
}

#[test]
fn vpsignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 8, 231], OperandSize::Dword)
}

#[test]
fn vpsignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 256563494, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 8, 52, 133, 38, 217, 74, 15], OperandSize::Dword)
}

#[test]
fn vpsignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 8, 203], OperandSize::Qword)
}

#[test]
fn vpsignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 8, 28, 187], OperandSize::Qword)
}

