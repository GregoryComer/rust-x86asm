use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 5, 220], OperandSize::Dword)
}

#[test]
fn vphsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 1347742721, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 5, 185, 1, 236, 84, 80], OperandSize::Dword)
}

#[test]
fn vphsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 5, 194], OperandSize::Qword)
}

#[test]
fn vphsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 331378567, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 5, 177, 135, 111, 192, 19], OperandSize::Qword)
}

#[test]
fn vphsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 5, 193], OperandSize::Dword)
}

#[test]
fn vphsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 353706987, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 5, 164, 152, 235, 35, 21, 21], OperandSize::Dword)
}

#[test]
fn vphsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 5, 237], OperandSize::Qword)
}

#[test]
fn vphsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 826783034, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 5, 4, 253, 58, 181, 71, 49], OperandSize::Qword)
}

