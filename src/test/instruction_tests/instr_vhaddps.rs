use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 124, 192], OperandSize::Dword)
}

#[test]
fn vhaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 369646390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 124, 28, 85, 54, 91, 8, 22], OperandSize::Dword)
}

#[test]
fn vhaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 124, 237], OperandSize::Qword)
}

#[test]
fn vhaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 298881624, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 124, 44, 85, 88, 146, 208, 17], OperandSize::Qword)
}

#[test]
fn vhaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 124, 197], OperandSize::Dword)
}

#[test]
fn vhaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 124, 36, 179], OperandSize::Dword)
}

#[test]
fn vhaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 239, 124, 243], OperandSize::Qword)
}

#[test]
fn vhaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 223, 124, 60, 178], OperandSize::Qword)
}

