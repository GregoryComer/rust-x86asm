use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermilpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 13, 240], OperandSize::Dword)
}

fn vpermilpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 13, 11], OperandSize::Dword)
}

fn vpermilpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 13, 254], OperandSize::Qword)
}

fn vpermilpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 69021088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 13, 180, 66, 160, 45, 29, 4], OperandSize::Qword)
}

fn vpermilpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 13, 245], OperandSize::Dword)
}

fn vpermilpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 13, 9], OperandSize::Dword)
}

fn vpermilpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 13, 239], OperandSize::Qword)
}

fn vpermilpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDX, 688366453, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 13, 178, 117, 163, 7, 41], OperandSize::Qword)
}

fn vpermilpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 13, 248], OperandSize::Dword)
}

fn vpermilpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 1399716859, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 13, 154, 251, 251, 109, 83], OperandSize::Dword)
}

fn vpermilpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 158, 13, 63], OperandSize::Dword)
}

fn vpermilpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 197, 134, 13, 212], OperandSize::Qword)
}

fn vpermilpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 197, 142, 13, 19], OperandSize::Qword)
}

fn vpermilpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RBX, 1437173120, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 157, 149, 13, 171, 128, 133, 169, 85], OperandSize::Qword)
}

fn vpermilpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 13, 230], OperandSize::Dword)
}

fn vpermilpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1376641166, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 13, 140, 134, 142, 224, 13, 82], OperandSize::Dword)
}

fn vpermilpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 188, 13, 12, 255], OperandSize::Dword)
}

fn vpermilpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 253, 163, 13, 249], OperandSize::Qword)
}

fn vpermilpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 189, 162, 13, 22], OperandSize::Qword)
}

fn vpermilpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RSI, 2010518046, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 149, 189, 13, 182, 30, 18, 214, 119], OperandSize::Qword)
}

fn vpermilpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 202, 13, 193], OperandSize::Dword)
}

fn vpermilpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 13, 4, 209], OperandSize::Dword)
}

fn vpermilpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1142498881, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 218, 13, 28, 141, 65, 38, 25, 68], OperandSize::Dword)
}

fn vpermilpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 205, 198, 13, 252], OperandSize::Qword)
}

fn vpermilpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 43051055, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 199, 13, 132, 112, 47, 232, 144, 2], OperandSize::Qword)
}

fn vpermilpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 13, 6], OperandSize::Qword)
}

fn vpermilpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 236, 70], OperandSize::Dword)
}

fn vpermilpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 1, 47], OperandSize::Dword)
}

fn vpermilpd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 230, 67], OperandSize::Qword)
}

fn vpermilpd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 603257947, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 60, 189, 91, 252, 244, 35, 38], OperandSize::Qword)
}

fn vpermilpd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 254, 85], OperandSize::Dword)
}

fn vpermilpd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EAX, 645603912, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 168, 72, 34, 123, 38, 100], OperandSize::Dword)
}

fn vpermilpd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 223, 76], OperandSize::Qword)
}

fn vpermilpd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 405727695, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 36, 157, 207, 233, 46, 24, 26], OperandSize::Qword)
}

fn vpermilpd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 142, 5, 227, 67], OperandSize::Dword)
}

fn vpermilpd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 5, 3, 60], OperandSize::Dword)
}

fn vpermilpd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 81042543, Some(OperandSize::Qword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 155, 5, 135, 111, 156, 212, 4, 19], OperandSize::Dword)
}

fn vpermilpd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 253, 138, 5, 244, 46], OperandSize::Qword)
}

fn vpermilpd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 986933743, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 142, 5, 28, 181, 239, 105, 211, 58, 75], OperandSize::Qword)
}

fn vpermilpd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 539921476, Some(OperandSize::Qword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 115, 253, 157, 5, 180, 185, 68, 140, 46, 32, 121], OperandSize::Qword)
}

fn vpermilpd_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 5, 228, 32], OperandSize::Dword)
}

fn vpermilpd_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 5, 36, 122, 64], OperandSize::Dword)
}

fn vpermilpd_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1185657612, Some(OperandSize::Qword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 189, 5, 180, 195, 12, 179, 171, 70, 95], OperandSize::Dword)
}

fn vpermilpd_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 253, 175, 5, 215, 32], OperandSize::Qword)
}

fn vpermilpd_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM26)), operand2: Some(IndirectDisplaced(RAX, 1141565171, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 253, 175, 5, 144, 243, 230, 10, 68, 40], OperandSize::Qword)
}

fn vpermilpd_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 191, 5, 4, 243, 52], OperandSize::Qword)
}

fn vpermilpd_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 5, 194, 79], OperandSize::Dword)
}

fn vpermilpd_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ECX, 154901245, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 202, 5, 169, 253, 154, 59, 9, 104], OperandSize::Dword)
}

fn vpermilpd_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1184424329, Some(OperandSize::Qword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 223, 5, 12, 149, 137, 225, 152, 70, 55], OperandSize::Dword)
}

fn vpermilpd_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 253, 205, 5, 236, 88], OperandSize::Qword)
}

fn vpermilpd_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 786170992, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 253, 202, 5, 12, 253, 112, 4, 220, 46, 108], OperandSize::Qword)
}

fn vpermilpd_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 586738492, Some(OperandSize::Qword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 253, 221, 5, 164, 179, 60, 235, 248, 34, 86], OperandSize::Qword)
}

