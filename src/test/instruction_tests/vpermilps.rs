use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermilps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 12, 247], OperandSize::Dword)
}

fn vpermilps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 12, 46], OperandSize::Dword)
}

fn vpermilps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 12, 227], OperandSize::Qword)
}

fn vpermilps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 1597283568, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 12, 167, 240, 156, 52, 95], OperandSize::Qword)
}

fn vpermilps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 12, 242], OperandSize::Dword)
}

fn vpermilps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 12, 63], OperandSize::Dword)
}

fn vpermilps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 12, 221], OperandSize::Qword)
}

fn vpermilps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 871535911, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 12, 132, 201, 39, 149, 242, 51], OperandSize::Qword)
}

fn vpermilps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 12, 234], OperandSize::Dword)
}

fn vpermilps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 503805673, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 12, 148, 208, 233, 118, 7, 30], OperandSize::Dword)
}

fn vpermilps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 158, 12, 30], OperandSize::Dword)
}

fn vpermilps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 29, 137, 12, 237], OperandSize::Qword)
}

fn vpermilps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 573588469, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 21, 139, 12, 188, 203, 245, 67, 48, 34], OperandSize::Qword)
}

fn vpermilps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 93, 146, 12, 56], OperandSize::Qword)
}

fn vpermilps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 171, 12, 251], OperandSize::Dword)
}

fn vpermilps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 12, 55], OperandSize::Dword)
}

fn vpermilps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 185, 12, 60, 81], OperandSize::Dword)
}

fn vpermilps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 109, 174, 12, 238], OperandSize::Qword)
}

fn vpermilps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RSI, 1456863306, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 109, 169, 12, 182, 74, 248, 213, 86], OperandSize::Qword)
}

fn vpermilps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 698646981, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 117, 189, 12, 188, 211, 197, 129, 164, 41], OperandSize::Qword)
}

fn vpermilps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 204, 12, 198], OperandSize::Dword)
}

fn vpermilps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 2090264846, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 12, 36, 245, 14, 233, 150, 124], OperandSize::Dword)
}

fn vpermilps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EBX, 299969413, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 221, 12, 131, 133, 43, 225, 17], OperandSize::Dword)
}

fn vpermilps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 29, 201, 12, 233], OperandSize::Qword)
}

fn vpermilps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 85, 196, 12, 11], OperandSize::Qword)
}

fn vpermilps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RCX, 713355108, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 85, 222, 12, 145, 100, 239, 132, 42], OperandSize::Qword)
}

fn vpermilps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 253, 15], OperandSize::Dword)
}

fn vpermilps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 33, 32], OperandSize::Dword)
}

fn vpermilps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 219, 5], OperandSize::Qword)
}

fn vpermilps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 36, 246, 68], OperandSize::Qword)
}

fn vpermilps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 251, 105], OperandSize::Dword)
}

fn vpermilps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1105139402, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 132, 159, 202, 22, 223, 65, 18], OperandSize::Dword)
}

fn vpermilps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 251, 57], OperandSize::Qword)
}

fn vpermilps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1282130369, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 52, 133, 193, 193, 107, 76, 76], OperandSize::Qword)
}

fn vpermilps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 4, 209, 48], OperandSize::Dword)
}

fn vpermilps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 4, 28, 80, 42], OperandSize::Dword)
}

fn vpermilps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 153, 4, 19, 75], OperandSize::Dword)
}

fn vpermilps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM18)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 51, 125, 141, 4, 242, 90], OperandSize::Qword)
}

fn vpermilps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 125, 139, 4, 60, 145, 107], OperandSize::Qword)
}

fn vpermilps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM17)), operand2: Some(IndirectDisplaced(RAX, 1300492585, Some(OperandSize::Dword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 125, 154, 4, 136, 41, 241, 131, 77, 102], OperandSize::Qword)
}

fn vpermilps_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 4, 230, 40], OperandSize::Dword)
}

fn vpermilps_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 4, 62, 21], OperandSize::Dword)
}

fn vpermilps_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1554164206, Some(OperandSize::Dword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 186, 4, 60, 221, 238, 169, 162, 92, 93], OperandSize::Dword)
}

fn vpermilps_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 125, 174, 4, 218, 91], OperandSize::Qword)
}

fn vpermilps_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 125, 173, 4, 52, 179, 63], OperandSize::Qword)
}

fn vpermilps_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1943104466, Some(OperandSize::Dword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 125, 189, 4, 140, 129, 210, 107, 209, 115, 127], OperandSize::Qword)
}

fn vpermilps_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 204, 4, 216, 68], OperandSize::Dword)
}

fn vpermilps_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EDX, 298974861, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 203, 4, 146, 141, 254, 209, 17, 71], OperandSize::Dword)
}

fn vpermilps_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 276512483, Some(OperandSize::Dword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 221, 4, 4, 197, 227, 62, 123, 16, 20], OperandSize::Dword)
}

fn vpermilps_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM10)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 195, 125, 202, 4, 226, 101], OperandSize::Qword)
}

fn vpermilps_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 125, 205, 4, 28, 144, 103], OperandSize::Qword)
}

fn vpermilps_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 125, 218, 4, 28, 179, 87], OperandSize::Qword)
}

