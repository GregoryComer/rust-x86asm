use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 126, 242], OperandSize::Dword)
}

#[test]
fn vpermt2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 580968986, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 126, 60, 189, 26, 226, 160, 34], OperandSize::Dword)
}

#[test]
fn vpermt2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 94478183, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 156, 126, 140, 243, 103, 159, 161, 5], OperandSize::Dword)
}

#[test]
fn vpermt2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 221, 130, 126, 247], OperandSize::Qword)
}

#[test]
fn vpermt2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1487903144, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 245, 132, 126, 36, 205, 168, 153, 175, 88], OperandSize::Qword)
}

#[test]
fn vpermt2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 141, 156, 126, 12, 206], OperandSize::Qword)
}

#[test]
fn vpermt2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 126, 234], OperandSize::Dword)
}

#[test]
fn vpermt2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 126, 28, 64], OperandSize::Dword)
}

#[test]
fn vpermt2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 137588276, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 191, 126, 44, 125, 52, 110, 51, 8], OperandSize::Dword)
}

#[test]
fn vpermt2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 133, 167, 126, 194], OperandSize::Qword)
}

#[test]
fn vpermt2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 884980295, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 141, 166, 126, 132, 91, 71, 186, 191, 52], OperandSize::Qword)
}

#[test]
fn vpermt2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 2105681098, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 181, 186, 126, 52, 213, 202, 36, 130, 125], OperandSize::Qword)
}

#[test]
fn vpermt2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 126, 209], OperandSize::Dword)
}

#[test]
fn vpermt2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1235637968, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 207, 126, 36, 213, 208, 86, 166, 73], OperandSize::Dword)
}

#[test]
fn vpermt2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 222, 126, 58], OperandSize::Dword)
}

#[test]
fn vpermt2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 173, 198, 126, 196], OperandSize::Qword)
}

#[test]
fn vpermt2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1424802788, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 221, 201, 126, 132, 146, 228, 195, 236, 84], OperandSize::Qword)
}

#[test]
fn vpermt2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 221, 214, 126, 1], OperandSize::Qword)
}

