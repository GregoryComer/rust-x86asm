use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangeps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 141, 80, 214, 115], OperandSize::Dword)
}

#[test]
fn vrangeps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 938182521, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 109, 137, 80, 164, 254, 121, 135, 235, 55, 39], OperandSize::Dword)
}

#[test]
fn vrangeps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1807605536, Some(OperandSize::Dword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 157, 80, 28, 117, 32, 223, 189, 107, 22], OperandSize::Dword)
}

#[test]
fn vrangeps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM20)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 163, 93, 143, 80, 244, 124], OperandSize::Qword)
}

#[test]
fn vrangeps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 208691649, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 77, 129, 80, 164, 150, 193, 97, 112, 12, 70], OperandSize::Qword)
}

#[test]
fn vrangeps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1294942499, Some(OperandSize::Dword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 21, 156, 80, 132, 243, 35, 65, 47, 77, 3], OperandSize::Qword)
}

#[test]
fn vrangeps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 170, 80, 214, 87], OperandSize::Dword)
}

#[test]
fn vrangeps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 171, 80, 4, 248, 59], OperandSize::Dword)
}

#[test]
fn vrangeps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 569962849, Some(OperandSize::Dword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 109, 191, 80, 132, 78, 97, 241, 248, 33, 62], OperandSize::Dword)
}

#[test]
fn vrangeps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM18)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 35, 69, 170, 80, 234, 76], OperandSize::Qword)
}

#[test]
fn vrangeps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 5, 165, 80, 42, 65], OperandSize::Qword)
}

#[test]
fn vrangeps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 109, 179, 80, 12, 191, 76], OperandSize::Qword)
}

#[test]
fn vrangeps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 157, 80, 245, 92], OperandSize::Dword)
}

#[test]
fn vrangeps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 85, 205, 80, 57, 82], OperandSize::Dword)
}

#[test]
fn vrangeps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 928684300, Some(OperandSize::Dword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 109, 222, 80, 180, 113, 12, 153, 90, 55, 73], OperandSize::Dword)
}

#[test]
fn vrangeps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM11)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 195, 53, 157, 80, 235, 18], OperandSize::Qword)
}

#[test]
fn vrangeps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1071679988, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 69, 197, 80, 36, 149, 244, 137, 224, 63, 15], OperandSize::Qword)
}

#[test]
fn vrangeps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 5, 220, 80, 38, 119], OperandSize::Qword)
}

