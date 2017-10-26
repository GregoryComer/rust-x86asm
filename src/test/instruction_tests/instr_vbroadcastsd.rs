use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 52, 86], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 88792930, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 148, 243, 98, 223, 74, 5], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 233], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 197], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 25, 219], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1656735205, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 25, 36, 253, 229, 197, 191, 98], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 253, 170, 25, 229], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1015457113, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 175, 25, 4, 117, 89, 165, 134, 60], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 25, 196], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDI, 550181670, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 25, 167, 38, 27, 203, 32], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 253, 207, 25, 254], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1337693008, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 25, 44, 245, 80, 147, 187, 79], OperandSize::Qword)
}

