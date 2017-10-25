use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 114, 195, 117], OperandSize::Dword)
}

#[test]
fn vprord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 950250393, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 114, 4, 245, 153, 171, 163, 56, 45], OperandSize::Dword)
}

#[test]
fn vprord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 492769213, Some(OperandSize::Dword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 156, 114, 4, 181, 189, 15, 95, 29, 74], OperandSize::Dword)
}

#[test]
fn vprord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM14)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 117, 135, 114, 198, 53], OperandSize::Qword)
}

#[test]
fn vprord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM23)), operand2: Some(IndirectDisplaced(RDI, 1924092599, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 129, 114, 135, 183, 82, 175, 114, 112], OperandSize::Qword)
}

#[test]
fn vprord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 308701947, Some(OperandSize::Dword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 154, 114, 4, 117, 251, 106, 102, 18, 126], OperandSize::Qword)
}

#[test]
fn vprord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 114, 195, 16], OperandSize::Dword)
}

#[test]
fn vprord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 114, 4, 158, 85], OperandSize::Dword)
}

#[test]
fn vprord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 77, 188, 114, 3, 77], OperandSize::Dword)
}

#[test]
fn vprord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 114, 196, 12], OperandSize::Qword)
}

#[test]
fn vprord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RDI, 1365085912, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 114, 135, 216, 142, 93, 81, 91], OperandSize::Qword)
}

#[test]
fn vprord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1139778777, Some(OperandSize::Dword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 182, 114, 4, 213, 217, 164, 239, 67, 4], OperandSize::Qword)
}

#[test]
fn vprord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 114, 194, 101], OperandSize::Dword)
}

#[test]
fn vprord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1918210242, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 206, 114, 4, 197, 194, 144, 85, 114, 5], OperandSize::Dword)
}

#[test]
fn vprord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1759685007, Some(OperandSize::Dword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 223, 114, 132, 83, 143, 169, 226, 104, 66], OperandSize::Dword)
}

#[test]
fn vprord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM22)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 53, 207, 114, 198, 30], OperandSize::Qword)
}

#[test]
fn vprord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1480599447, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 193, 114, 4, 205, 151, 39, 64, 88, 119], OperandSize::Qword)
}

#[test]
fn vprord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM10)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 45, 221, 114, 3, 111], OperandSize::Qword)
}

