use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 114, 210, 99], OperandSize::Dword)
}

fn vpsrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 114, 212, 33], OperandSize::Qword)
}

fn vpsrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 114, 212, 55], OperandSize::Dword)
}

fn vpsrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 114, 210, 98], OperandSize::Qword)
}

fn vpsrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 114, 209, 46], OperandSize::Dword)
}

fn vpsrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 114, 22, 80], OperandSize::Dword)
}

fn vpsrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 399495864, Some(OperandSize::Dword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 156, 114, 144, 184, 210, 207, 23, 12], OperandSize::Dword)
}

fn vpsrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 125, 142, 114, 208, 91], OperandSize::Qword)
}

fn vpsrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 143, 114, 20, 250, 117], OperandSize::Qword)
}

fn vpsrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 49871627, Some(OperandSize::Dword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 21, 154, 114, 20, 213, 11, 251, 248, 2, 57], OperandSize::Qword)
}

fn vpsrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 114, 211, 113], OperandSize::Dword)
}

fn vpsrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EBX, 1842078631, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 114, 147, 167, 227, 203, 109, 68], OperandSize::Dword)
}

fn vpsrld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 187, 114, 22, 44], OperandSize::Dword)
}

fn vpsrld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 93, 172, 114, 213, 11], OperandSize::Qword)
}

fn vpsrld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 5, 169, 114, 20, 64, 19], OperandSize::Qword)
}

fn vpsrld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM17)), operand2: Some(IndirectDisplaced(RBX, 1496860799, Some(OperandSize::Dword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 183, 114, 147, 127, 72, 56, 89, 32], OperandSize::Qword)
}

fn vpsrld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 206, 114, 210, 56], OperandSize::Dword)
}

fn vpsrld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 114, 17, 103], OperandSize::Dword)
}

fn vpsrld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 222, 114, 20, 192, 29], OperandSize::Dword)
}

fn vpsrld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM9)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 109, 198, 114, 209, 0], OperandSize::Qword)
}

fn vpsrld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 29, 204, 114, 20, 184, 124], OperandSize::Qword)
}

fn vpsrld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1273579125, Some(OperandSize::Dword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 61, 219, 114, 20, 197, 117, 70, 233, 75, 94], OperandSize::Qword)
}

fn vpsrld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 210, 247], OperandSize::Dword)
}

fn vpsrld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 2059400487, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 210, 144, 39, 245, 191, 122], OperandSize::Dword)
}

fn vpsrld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 210, 244], OperandSize::Qword)
}

fn vpsrld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 210, 20, 200], OperandSize::Qword)
}

fn vpsrld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 210, 253], OperandSize::Dword)
}

fn vpsrld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 210, 44, 121], OperandSize::Dword)
}

fn vpsrld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 210, 248], OperandSize::Qword)
}

fn vpsrld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 2094460936, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 210, 156, 249, 8, 240, 214, 124], OperandSize::Qword)
}

fn vpsrld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 210, 246], OperandSize::Dword)
}

fn vpsrld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 384188851, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 210, 60, 157, 179, 65, 230, 22], OperandSize::Dword)
}

fn vpsrld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 45, 129, 210, 252], OperandSize::Qword)
}

fn vpsrld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1760221362, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 125, 138, 210, 180, 147, 178, 216, 234, 104], OperandSize::Qword)
}

fn vpsrld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 210, 236], OperandSize::Dword)
}

fn vpsrld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 210, 44, 121], OperandSize::Dword)
}

fn vpsrld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 61, 171, 210, 200], OperandSize::Qword)
}

fn vpsrld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 61, 172, 210, 23], OperandSize::Qword)
}

fn vpsrld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 203, 210, 211], OperandSize::Dword)
}

fn vpsrld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 2142054308, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 210, 60, 133, 164, 39, 173, 127], OperandSize::Dword)
}

fn vpsrld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 117, 194, 210, 236], OperandSize::Qword)
}

fn vpsrld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1818493579, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 29, 199, 210, 156, 87, 139, 2, 100, 108], OperandSize::Qword)
}

