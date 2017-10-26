use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 93, 206], OperandSize::Dword)
}

#[test]
fn vminpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 93, 58], OperandSize::Dword)
}

#[test]
fn vminpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 93, 224], OperandSize::Qword)
}

#[test]
fn vminpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 93, 60, 218], OperandSize::Qword)
}

#[test]
fn vminpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 93, 199], OperandSize::Dword)
}

#[test]
fn vminpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 93, 12, 183], OperandSize::Dword)
}

#[test]
fn vminpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 93, 228], OperandSize::Qword)
}

#[test]
fn vminpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1230117928, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 93, 188, 240, 40, 28, 82, 73], OperandSize::Qword)
}

#[test]
fn vminpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 137, 93, 221], OperandSize::Dword)
}

#[test]
fn vminpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1554482198, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 93, 188, 134, 22, 132, 167, 92], OperandSize::Dword)
}

#[test]
fn vminpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1409079203, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 154, 93, 172, 147, 163, 215, 252, 83], OperandSize::Dword)
}

#[test]
fn vminpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 237, 135, 93, 242], OperandSize::Qword)
}

#[test]
fn vminpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 900816136, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 237, 141, 93, 60, 69, 8, 93, 177, 53], OperandSize::Qword)
}

#[test]
fn vminpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 189, 158, 93, 4, 190], OperandSize::Qword)
}

#[test]
fn vminpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 172, 93, 240], OperandSize::Dword)
}

#[test]
fn vminpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 658143518, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 172, 93, 166, 30, 121, 58, 39], OperandSize::Dword)
}

#[test]
fn vminpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 190, 93, 20, 131], OperandSize::Dword)
}

#[test]
fn vminpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 141, 174, 93, 254], OperandSize::Qword)
}

#[test]
fn vminpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 661371733, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 189, 164, 93, 36, 149, 85, 187, 107, 39], OperandSize::Qword)
}

#[test]
fn vminpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 205, 177, 93, 20, 200], OperandSize::Qword)
}

#[test]
fn vminpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 153, 93, 217], OperandSize::Dword)
}

#[test]
fn vminpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1667770995, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 206, 93, 4, 117, 115, 42, 104, 99], OperandSize::Dword)
}

#[test]
fn vminpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 223, 93, 52, 136], OperandSize::Dword)
}

#[test]
fn vminpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 253, 150, 93, 246], OperandSize::Qword)
}

#[test]
fn vminpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 245, 202, 93, 2], OperandSize::Qword)
}

#[test]
fn vminpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectDisplaced(RAX, 1797627755, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 141, 218, 93, 128, 107, 159, 37, 107], OperandSize::Qword)
}

