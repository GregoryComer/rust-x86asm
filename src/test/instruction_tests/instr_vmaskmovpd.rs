use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 45, 14], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 490867490, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 45, 60, 117, 34, 11, 66, 29], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 195944453, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 45, 52, 69, 5, 224, 173, 11], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RCX, 1895483033, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 45, 169, 153, 198, 250, 112], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(ESI, Four, 2076251763, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 47, 52, 181, 115, 22, 193, 123], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 47, 58], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 47, 60, 223], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 47, 43], OperandSize::Qword)
}

