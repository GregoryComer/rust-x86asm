use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 92, 240], OperandSize::Dword)
}

#[test]
fn vsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 92, 43], OperandSize::Dword)
}

#[test]
fn vsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 92, 238], OperandSize::Qword)
}

#[test]
fn vsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 92, 60, 242], OperandSize::Qword)
}

#[test]
fn vsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 92, 253], OperandSize::Dword)
}

#[test]
fn vsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 92, 18], OperandSize::Dword)
}

#[test]
fn vsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 92, 199], OperandSize::Qword)
}

#[test]
fn vsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 92, 35], OperandSize::Qword)
}

#[test]
fn vsubps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 92, 211], OperandSize::Dword)
}

#[test]
fn vsubps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 76, 142, 92, 20, 65], OperandSize::Dword)
}

#[test]
fn vsubps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 157, 92, 42], OperandSize::Dword)
}

#[test]
fn vsubps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 12, 131, 92, 200], OperandSize::Qword)
}

#[test]
fn vsubps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 20, 134, 92, 12, 153], OperandSize::Qword)
}

#[test]
fn vsubps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 76, 148, 92, 60, 185], OperandSize::Qword)
}

#[test]
fn vsubps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 175, 92, 216], OperandSize::Dword)
}

#[test]
fn vsubps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1675981403, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 171, 92, 36, 93, 91, 114, 229, 99], OperandSize::Dword)
}

#[test]
fn vsubps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 410322469, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 188, 92, 135, 37, 6, 117, 24], OperandSize::Dword)
}

#[test]
fn vsubps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 116, 167, 92, 212], OperandSize::Qword)
}

#[test]
fn vsubps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 84, 172, 92, 52, 112], OperandSize::Qword)
}

#[test]
fn vsubps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 711385783, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 28, 188, 92, 188, 199, 183, 226, 102, 42], OperandSize::Qword)
}

#[test]
fn vsubps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 249, 92, 226], OperandSize::Dword)
}

#[test]
fn vsubps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 147234786, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 205, 92, 178, 226, 159, 198, 8], OperandSize::Dword)
}

#[test]
fn vsubps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 217, 92, 43], OperandSize::Dword)
}

#[test]
fn vsubps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 52, 247, 92, 214], OperandSize::Qword)
}

#[test]
fn vsubps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 198, 92, 35], OperandSize::Qword)
}

#[test]
fn vsubps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 214, 92, 51], OperandSize::Qword)
}

