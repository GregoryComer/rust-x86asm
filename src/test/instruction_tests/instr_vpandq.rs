use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 137, 219, 222], OperandSize::Dword)
}

#[test]
fn vpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 219, 16], OperandSize::Dword)
}

#[test]
fn vpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 460187346, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 158, 219, 20, 141, 210, 230, 109, 27], OperandSize::Dword)
}

#[test]
fn vpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 173, 143, 219, 232], OperandSize::Qword)
}

#[test]
fn vpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RDX, 1264411621, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 173, 139, 219, 130, 229, 99, 93, 75], OperandSize::Qword)
}

#[test]
fn vpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RDX, 898826638, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 253, 146, 219, 146, 142, 1, 147, 53], OperandSize::Qword)
}

#[test]
fn vpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 174, 219, 194], OperandSize::Dword)
}

#[test]
fn vpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1661065029, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 175, 219, 132, 182, 69, 215, 1, 99], OperandSize::Dword)
}

#[test]
fn vpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 112490506, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 190, 219, 44, 69, 10, 120, 180, 6], OperandSize::Dword)
}

#[test]
fn vpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 245, 172, 219, 195], OperandSize::Qword)
}

#[test]
fn vpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RCX, 1910737326, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 170, 219, 177, 174, 137, 227, 113], OperandSize::Qword)
}

#[test]
fn vpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2022347043, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 245, 182, 219, 52, 221, 35, 145, 138, 120], OperandSize::Qword)
}

#[test]
fn vpandq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 203, 219, 212], OperandSize::Dword)
}

#[test]
fn vpandq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 207, 219, 14], OperandSize::Dword)
}

#[test]
fn vpandq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 223, 219, 57], OperandSize::Dword)
}

#[test]
fn vpandq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 221, 199, 219, 212], OperandSize::Qword)
}

#[test]
fn vpandq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 157, 206, 219, 31], OperandSize::Qword)
}

#[test]
fn vpandq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 157, 217, 219, 38], OperandSize::Qword)
}

