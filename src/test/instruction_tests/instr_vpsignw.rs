use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 9, 254], OperandSize::Dword)
}

#[test]
fn vpsignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 9, 51], OperandSize::Dword)
}

#[test]
fn vpsignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 9, 195], OperandSize::Qword)
}

#[test]
fn vpsignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 425302058, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 9, 190, 42, 152, 89, 25], OperandSize::Qword)
}

#[test]
fn vpsignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 9, 238], OperandSize::Dword)
}

#[test]
fn vpsignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 9, 44, 151], OperandSize::Dword)
}

#[test]
fn vpsignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 9, 202], OperandSize::Qword)
}

#[test]
fn vpsignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 9, 42], OperandSize::Qword)
}

