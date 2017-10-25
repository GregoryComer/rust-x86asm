use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 64, 234, 99], OperandSize::Dword)
}

#[test]
fn vdpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1831356317, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 64, 140, 88, 157, 71, 40, 109, 120], OperandSize::Dword)
}

#[test]
fn vdpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 64, 251, 43], OperandSize::Qword)
}

#[test]
fn vdpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 64, 12, 195, 41], OperandSize::Qword)
}

#[test]
fn vdpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 64, 218, 2], OperandSize::Dword)
}

#[test]
fn vdpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1539560863, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 64, 12, 253, 159, 213, 195, 91, 52], OperandSize::Dword)
}

#[test]
fn vdpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 64, 255, 41], OperandSize::Qword)
}

#[test]
fn vdpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 64, 46, 90], OperandSize::Qword)
}

