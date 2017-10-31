use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 87, 214], OperandSize::Dword)
}

#[test]
fn vxorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 758121922, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 87, 156, 91, 194, 5, 48, 45], OperandSize::Dword)
}

#[test]
fn vxorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 87, 229], OperandSize::Qword)
}

#[test]
fn vxorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1259903700, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 87, 156, 130, 212, 154, 24, 75], OperandSize::Qword)
}

#[test]
fn vxorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 87, 194], OperandSize::Dword)
}

#[test]
fn vxorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1789980157, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 87, 60, 181, 253, 237, 176, 106], OperandSize::Dword)
}

#[test]
fn vxorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 87, 204], OperandSize::Qword)
}

#[test]
fn vxorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 87, 12, 67], OperandSize::Qword)
}

#[test]
fn vxorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 87, 231], OperandSize::Dword)
}

#[test]
fn vxorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1896651793, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 92, 139, 87, 44, 253, 17, 156, 12, 113], OperandSize::Dword)
}

#[test]
fn vxorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1832016255, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 156, 87, 147, 127, 89, 50, 109], OperandSize::Dword)
}

#[test]
fn vxorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 68, 134, 87, 240], OperandSize::Qword)
}

#[test]
fn vxorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 2075509420, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 84, 132, 87, 36, 77, 172, 194, 181, 123], OperandSize::Qword)
}

#[test]
fn vxorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RDI, 1860788160, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 44, 159, 87, 151, 192, 95, 233, 110], OperandSize::Qword)
}

#[test]
fn vxorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 174, 87, 203], OperandSize::Dword)
}

#[test]
fn vxorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1503597610, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 171, 87, 60, 245, 42, 20, 159, 89], OperandSize::Dword)
}

#[test]
fn vxorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 187, 87, 28, 146], OperandSize::Dword)
}

#[test]
fn vxorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 4, 165, 87, 222], OperandSize::Qword)
}

#[test]
fn vxorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 12, 164, 87, 44, 129], OperandSize::Qword)
}

#[test]
fn vxorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 182, 87, 4, 211], OperandSize::Qword)
}

#[test]
fn vxorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 207, 87, 198], OperandSize::Dword)
}

#[test]
fn vxorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 84205001, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 116, 206, 87, 132, 207, 201, 221, 4, 5], OperandSize::Dword)
}

#[test]
fn vxorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 277176846, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 223, 87, 44, 205, 14, 98, 133, 16], OperandSize::Dword)
}

#[test]
fn vxorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 116, 207, 87, 218], OperandSize::Qword)
}

#[test]
fn vxorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RSI, 1887453692, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 28, 196, 87, 174, 252, 65, 128, 112], OperandSize::Qword)
}

#[test]
fn vxorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM30)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 12, 214, 87, 23], OperandSize::Qword)
}

