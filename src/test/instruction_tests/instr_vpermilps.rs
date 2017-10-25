use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 12, 216], OperandSize::Dword)
}

#[test]
fn vpermilps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 522964530, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 12, 44, 133, 50, 206, 43, 31], OperandSize::Dword)
}

#[test]
fn vpermilps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 12, 211], OperandSize::Qword)
}

#[test]
fn vpermilps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 12, 44, 176], OperandSize::Qword)
}

#[test]
fn vpermilps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 12, 253], OperandSize::Dword)
}

#[test]
fn vpermilps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 611091793, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 12, 12, 205, 81, 133, 108, 36], OperandSize::Dword)
}

#[test]
fn vpermilps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 12, 220], OperandSize::Qword)
}

#[test]
fn vpermilps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RSI, 1401856867, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 12, 150, 99, 163, 142, 83], OperandSize::Qword)
}

#[test]
fn vpermilps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 12, 247], OperandSize::Dword)
}

#[test]
fn vpermilps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 561610511, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 12, 166, 15, 127, 121, 33], OperandSize::Dword)
}

#[test]
fn vpermilps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 1334616808, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 156, 12, 188, 82, 232, 162, 140, 79], OperandSize::Dword)
}

#[test]
fn vpermilps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 77, 133, 12, 238], OperandSize::Qword)
}

#[test]
fn vpermilps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 5, 141, 12, 50], OperandSize::Qword)
}

#[test]
fn vpermilps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 45, 150, 12, 24], OperandSize::Qword)
}

#[test]
fn vpermilps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 12, 255], OperandSize::Dword)
}

#[test]
fn vpermilps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 12, 36, 240], OperandSize::Dword)
}

#[test]
fn vpermilps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 258788571, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 187, 12, 134, 219, 204, 108, 15], OperandSize::Dword)
}

#[test]
fn vpermilps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 117, 172, 12, 253], OperandSize::Qword)
}

#[test]
fn vpermilps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 888190666, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 172, 12, 12, 197, 202, 182, 240, 52], OperandSize::Qword)
}

#[test]
fn vpermilps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 879332601, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 37, 186, 12, 140, 206, 249, 140, 105, 52], OperandSize::Qword)
}

#[test]
fn vpermilps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 12, 194], OperandSize::Dword)
}

#[test]
fn vpermilps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1317804486, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 12, 52, 197, 198, 25, 140, 78], OperandSize::Dword)
}

#[test]
fn vpermilps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2124456410, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 220, 12, 28, 205, 218, 161, 160, 126], OperandSize::Dword)
}

#[test]
fn vpermilps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 5, 196, 12, 249], OperandSize::Qword)
}

#[test]
fn vpermilps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1522782751, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 45, 195, 12, 172, 179, 31, 210, 195, 90], OperandSize::Qword)
}

#[test]
fn vpermilps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RBX, 2124633196, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 13, 211, 12, 131, 108, 84, 163, 126], OperandSize::Qword)
}

#[test]
fn vpermilps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 202, 15], OperandSize::Dword)
}

#[test]
fn vpermilps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 1880940247, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 129, 215, 222, 28, 112, 45], OperandSize::Dword)
}

#[test]
fn vpermilps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 246, 4], OperandSize::Qword)
}

#[test]
fn vpermilps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1384805225, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 60, 245, 105, 115, 138, 82, 110], OperandSize::Qword)
}

#[test]
fn vpermilps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 206, 13], OperandSize::Dword)
}

#[test]
fn vpermilps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 737886597, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 140, 134, 133, 65, 251, 43, 11], OperandSize::Dword)
}

#[test]
fn vpermilps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 206, 91], OperandSize::Qword)
}

#[test]
fn vpermilps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 25, 68], OperandSize::Qword)
}

#[test]
fn vpermilps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 4, 211, 56], OperandSize::Dword)
}

#[test]
fn vpermilps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 142, 4, 36, 73, 6], OperandSize::Dword)
}

#[test]
fn vpermilps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 153, 4, 35, 10], OperandSize::Dword)
}

#[test]
fn vpermilps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM27)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 3, 125, 137, 4, 243, 10], OperandSize::Qword)
}

#[test]
fn vpermilps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RAX, 1332079148, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 125, 139, 4, 136, 44, 234, 101, 79, 87], OperandSize::Qword)
}

#[test]
fn vpermilps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1626730523, Some(OperandSize::Dword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 125, 157, 4, 44, 181, 27, 240, 245, 96, 11], OperandSize::Qword)
}

#[test]
fn vpermilps_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 172, 4, 213, 122], OperandSize::Dword)
}

#[test]
fn vpermilps_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 353991640, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 4, 156, 83, 216, 123, 25, 21, 84], OperandSize::Dword)
}

#[test]
fn vpermilps_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 191, 4, 26, 107], OperandSize::Dword)
}

#[test]
fn vpermilps_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM29)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 19, 125, 172, 4, 205, 81], OperandSize::Qword)
}

#[test]
fn vpermilps_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1049607722, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 125, 172, 4, 172, 242, 42, 190, 143, 62, 32], OperandSize::Qword)
}

#[test]
fn vpermilps_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RDX, 1530679554, Some(OperandSize::Dword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 190, 4, 178, 2, 81, 60, 91, 114], OperandSize::Qword)
}

#[test]
fn vpermilps_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 4, 216, 9], OperandSize::Dword)
}

#[test]
fn vpermilps_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1122711336, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 202, 4, 12, 141, 40, 55, 235, 66, 54], OperandSize::Dword)
}

#[test]
fn vpermilps_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 223, 4, 31, 104], OperandSize::Dword)
}

#[test]
fn vpermilps_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 131, 125, 205, 4, 230, 22], OperandSize::Qword)
}

#[test]
fn vpermilps_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 125, 205, 4, 34, 42], OperandSize::Qword)
}

#[test]
fn vpermilps_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 190241273, Some(OperandSize::Dword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 222, 4, 172, 134, 249, 217, 86, 11, 91], OperandSize::Qword)
}

