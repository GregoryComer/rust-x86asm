use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 114, 246, 114], OperandSize::Dword)
}

fn vpslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 114, 240, 113], OperandSize::Qword)
}

fn vpslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 114, 244, 50], OperandSize::Dword)
}

fn vpslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 114, 243, 15], OperandSize::Qword)
}

fn vpslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 114, 246, 45], OperandSize::Dword)
}

fn vpslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1260214271, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 140, 114, 52, 93, 255, 87, 29, 75, 63], OperandSize::Dword)
}

fn vpslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 899766987, Some(OperandSize::Dword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 158, 114, 180, 72, 203, 90, 161, 53, 96], OperandSize::Dword)
}

fn vpslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM14)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 101, 140, 114, 246, 51], OperandSize::Qword)
}

fn vpslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 114, 51, 51], OperandSize::Qword)
}

fn vpslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM24)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 61, 151, 114, 51, 53], OperandSize::Qword)
}

fn vpslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 114, 244, 34], OperandSize::Dword)
}

fn vpslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1717599292, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 114, 52, 149, 60, 124, 96, 102, 44], OperandSize::Dword)
}

fn vpslld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1998615353, Some(OperandSize::Dword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 190, 114, 180, 207, 57, 115, 32, 119, 62], OperandSize::Dword)
}

fn vpslld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM29)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 109, 174, 114, 245, 39], OperandSize::Qword)
}

fn vpslld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 748034442, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 114, 180, 122, 138, 25, 150, 44, 35], OperandSize::Qword)
}

fn vpslld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 596997168, Some(OperandSize::Dword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 37, 191, 114, 52, 77, 48, 116, 149, 35, 86], OperandSize::Qword)
}

fn vpslld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 114, 241, 8], OperandSize::Dword)
}

fn vpslld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1877007847, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 114, 180, 73, 231, 221, 224, 111, 89], OperandSize::Dword)
}

fn vpslld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 117, 221, 114, 49, 114], OperandSize::Dword)
}

fn vpslld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM9)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 5, 201, 114, 241, 101], OperandSize::Qword)
}

fn vpslld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1521861210, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 195, 114, 180, 126, 90, 194, 181, 90, 88], OperandSize::Qword)
}

fn vpslld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1920531545, Some(OperandSize::Dword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 45, 213, 114, 180, 190, 89, 252, 120, 114, 54], OperandSize::Qword)
}

fn vpslld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 242, 215], OperandSize::Dword)
}

fn vpslld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 242, 9], OperandSize::Dword)
}

fn vpslld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 242, 243], OperandSize::Qword)
}

fn vpslld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 242, 40], OperandSize::Qword)
}

fn vpslld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 242, 218], OperandSize::Dword)
}

fn vpslld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 242, 52, 202], OperandSize::Dword)
}

fn vpslld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 242, 239], OperandSize::Qword)
}

fn vpslld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 938634565, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 242, 147, 69, 109, 242, 55], OperandSize::Qword)
}

fn vpslld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 242, 229], OperandSize::Dword)
}

fn vpslld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 829266074, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 242, 164, 194, 154, 152, 109, 49], OperandSize::Dword)
}

fn vpslld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 125, 142, 242, 235], OperandSize::Qword)
}

fn vpslld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RCX, 121122502, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 101, 142, 242, 161, 198, 46, 56, 7], OperandSize::Qword)
}

fn vpslld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 242, 192], OperandSize::Dword)
}

fn vpslld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 1463260500, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 242, 147, 84, 149, 55, 87], OperandSize::Dword)
}

fn vpslld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 242, 196], OperandSize::Qword)
}

fn vpslld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1283313700, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 61, 165, 242, 20, 117, 36, 208, 125, 76], OperandSize::Qword)
}

fn vpslld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 242, 207], OperandSize::Dword)
}

fn vpslld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 242, 4, 178], OperandSize::Dword)
}

fn vpslld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 61, 194, 242, 249], OperandSize::Qword)
}

fn vpslld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1530553229, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 206, 242, 52, 157, 141, 99, 58, 91], OperandSize::Qword)
}

