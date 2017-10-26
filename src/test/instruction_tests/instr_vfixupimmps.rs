use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 109, 142, 84, 209, 8], OperandSize::Dword)
}

#[test]
fn vfixupimmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 84, 3, 41], OperandSize::Dword)
}

#[test]
fn vfixupimmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 253946448, Some(OperandSize::Dword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 159, 84, 156, 150, 80, 234, 34, 15, 63], OperandSize::Dword)
}

#[test]
fn vfixupimmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM15)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 83, 61, 131, 84, 255, 110], OperandSize::Qword)
}

#[test]
fn vfixupimmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 518283611, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 29, 139, 84, 156, 209, 91, 97, 228, 30, 89], OperandSize::Qword)
}

#[test]
fn vfixupimmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 45, 145, 84, 34, 8], OperandSize::Qword)
}

#[test]
fn vfixupimmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 173, 84, 214, 111], OperandSize::Dword)
}

#[test]
fn vfixupimmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 174, 84, 36, 241, 66], OperandSize::Dword)
}

#[test]
fn vfixupimmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 365416380, Some(OperandSize::Dword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 85, 189, 84, 140, 126, 188, 207, 199, 21, 81], OperandSize::Dword)
}

#[test]
fn vfixupimmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM23)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 35, 53, 169, 84, 239, 8], OperandSize::Qword)
}

#[test]
fn vfixupimmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1637020266, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 5, 163, 84, 20, 221, 106, 242, 146, 97, 37], OperandSize::Qword)
}

#[test]
fn vfixupimmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1570982847, Some(OperandSize::Dword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 5, 190, 84, 12, 77, 191, 75, 163, 93, 113], OperandSize::Qword)
}

#[test]
fn vfixupimmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 117, 155, 84, 238, 102], OperandSize::Dword)
}

#[test]
fn vfixupimmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 201, 84, 52, 112, 84], OperandSize::Dword)
}

#[test]
fn vfixupimmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDX, 1291132461, Some(OperandSize::Dword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 218, 84, 154, 45, 30, 245, 76, 45], OperandSize::Dword)
}

#[test]
fn vfixupimmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM27)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 93, 149, 84, 235, 62], OperandSize::Qword)
}

#[test]
fn vfixupimmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RSI, 92493214, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 101, 201, 84, 150, 158, 85, 131, 5, 43], OperandSize::Qword)
}

#[test]
fn vfixupimmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 13, 217, 84, 44, 159, 71], OperandSize::Qword)
}

