use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 644400092, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 148, 121, 220, 195, 104, 38], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1225976203, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 188, 119, 139, 233, 18, 73], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 236], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 202], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 25, 215], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 803420314, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 25, 4, 181, 154, 56, 227, 47], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 253, 171, 25, 215], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 722176286, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 253, 175, 25, 60, 69, 30, 137, 11, 43], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 25, 241], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1706645064, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 25, 4, 125, 72, 86, 185, 101], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 25, 248], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 104598055, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 201, 25, 148, 134, 39, 10, 60, 6], OperandSize::Qword)
}

