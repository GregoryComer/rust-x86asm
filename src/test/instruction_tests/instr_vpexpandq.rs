use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 137, 211], OperandSize::Dword)
}

#[test]
fn vpexpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ESI, 1498408234, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 137, 166, 42, 229, 79, 89], OperandSize::Dword)
}

#[test]
fn vpexpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 253, 138, 137, 197], OperandSize::Qword)
}

#[test]
fn vpexpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM12)), operand2: Some(IndirectDisplaced(RCX, 629354644, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 253, 140, 137, 161, 148, 48, 131, 37], OperandSize::Qword)
}

#[test]
fn vpexpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 137, 220], OperandSize::Dword)
}

#[test]
fn vpexpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 2132962135, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 137, 156, 201, 87, 107, 34, 127], OperandSize::Dword)
}

#[test]
fn vpexpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 253, 171, 137, 255], OperandSize::Qword)
}

#[test]
fn vpexpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 865033211, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 137, 36, 117, 251, 91, 143, 51], OperandSize::Qword)
}

#[test]
fn vpexpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 137, 232], OperandSize::Dword)
}

#[test]
fn vpexpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 348506181, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 137, 36, 197, 69, 200, 197, 20], OperandSize::Dword)
}

#[test]
fn vpexpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 253, 201, 137, 197], OperandSize::Qword)
}

#[test]
fn vpexpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 203, 137, 60, 206], OperandSize::Qword)
}

