use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 113, 247, 4], OperandSize::Dword)
}

fn vpsllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 113, 245, 116], OperandSize::Qword)
}

fn vpsllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 113, 247, 51], OperandSize::Dword)
}

fn vpsllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 113, 247, 112], OperandSize::Qword)
}

fn vpsllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 113, 242, 107], OperandSize::Dword)
}

fn vpsllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1453814957, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 141, 113, 52, 125, 173, 116, 167, 86, 115], OperandSize::Dword)
}

fn vpsllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 37, 132, 113, 244, 112], OperandSize::Qword)
}

fn vpsllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RDX, 621542343, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 53, 142, 113, 178, 199, 251, 11, 37, 115], OperandSize::Qword)
}

fn vpsllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 113, 244, 86], OperandSize::Dword)
}

fn vpsllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1497875152, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 172, 113, 180, 71, 208, 194, 71, 89, 29], OperandSize::Dword)
}

fn vpsllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM25)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 21, 170, 113, 241, 88], OperandSize::Qword)
}

fn vpsllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 2061030466, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 37, 173, 113, 180, 222, 66, 212, 216, 122, 68], OperandSize::Qword)
}

fn vpsllw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 113, 241, 49], OperandSize::Dword)
}

fn vpsllw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1296196664, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 113, 180, 242, 56, 100, 66, 77, 122], OperandSize::Dword)
}

fn vpsllw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM18)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 37, 205, 113, 242, 123], OperandSize::Qword)
}

fn vpsllw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1638205987, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 21, 197, 113, 52, 93, 35, 10, 165, 97, 39], OperandSize::Qword)
}

fn vpsllw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 241, 243], OperandSize::Dword)
}

fn vpsllw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 241, 36, 135], OperandSize::Dword)
}

fn vpsllw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 241, 212], OperandSize::Qword)
}

fn vpsllw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 977993299, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 241, 180, 66, 83, 254, 74, 58], OperandSize::Qword)
}

fn vpsllw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 241, 196], OperandSize::Dword)
}

fn vpsllw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 63636730, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 241, 163, 250, 4, 203, 3], OperandSize::Dword)
}

fn vpsllw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 241, 231], OperandSize::Qword)
}

fn vpsllw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 93774593, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 241, 44, 93, 1, 227, 150, 5], OperandSize::Qword)
}

fn vpsllw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 241, 215], OperandSize::Dword)
}

fn vpsllw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1253732153, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 241, 44, 181, 57, 111, 186, 74], OperandSize::Dword)
}

fn vpsllw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 61, 143, 241, 230], OperandSize::Qword)
}

fn vpsllw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RDI, 1816488839, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 143, 241, 167, 135, 107, 69, 108], OperandSize::Qword)
}

fn vpsllw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 241, 195], OperandSize::Dword)
}

fn vpsllw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 241, 44, 131], OperandSize::Dword)
}

fn vpsllw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 45, 174, 241, 201], OperandSize::Qword)
}

fn vpsllw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1798915287, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 45, 167, 241, 4, 205, 215, 68, 57, 107], OperandSize::Qword)
}

fn vpsllw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 241, 227], OperandSize::Dword)
}

fn vpsllw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1998766599, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 241, 36, 149, 7, 194, 34, 119], OperandSize::Dword)
}

fn vpsllw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 29, 194, 241, 218], OperandSize::Qword)
}

fn vpsllw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 987439827, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 197, 241, 28, 205, 211, 34, 219, 58], OperandSize::Qword)
}

