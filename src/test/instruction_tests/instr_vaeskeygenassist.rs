use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 218, 93], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1280267789, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 156, 75, 13, 86, 79, 76, 12], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 227, 62], OperandSize::Qword)
}

#[test]
fn vaeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1273356066, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 140, 151, 34, 223, 229, 75, 3], OperandSize::Qword)
}

