use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaskmovpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 45, 12, 129], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1497313085, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 45, 4, 189, 61, 47, 63, 89], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 45, 20, 222], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RBX, 1229518277, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 45, 171, 197, 245, 72, 73], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1274572009, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 47, 52, 125, 233, 108, 248, 75], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 999559455, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 47, 12, 205, 31, 17, 148, 59], OperandSize::Qword)
}

#[test]
fn vmaskmovpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 47, 35], OperandSize::Dword)
}

#[test]
fn vmaskmovpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMASKMOVPD, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 147942797, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 47, 44, 253, 141, 109, 209, 8], OperandSize::Qword)
}

