use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 114, 209, 55], OperandSize::Dword)
}

#[test]
fn vpsrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 114, 209, 94], OperandSize::Qword)
}

#[test]
fn vpsrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 114, 210, 22], OperandSize::Dword)
}

#[test]
fn vpsrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 114, 210, 82], OperandSize::Qword)
}

#[test]
fn vpsrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 114, 213, 33], OperandSize::Dword)
}

#[test]
fn vpsrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 899135718, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 140, 114, 20, 181, 230, 184, 151, 53, 31], OperandSize::Dword)
}

#[test]
fn vpsrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 158, 114, 20, 177, 117], OperandSize::Dword)
}

#[test]
fn vpsrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 29, 135, 114, 210, 50], OperandSize::Qword)
}

#[test]
fn vpsrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 543751195, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 132, 114, 148, 191, 27, 252, 104, 32, 4], OperandSize::Qword)
}

#[test]
fn vpsrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1612375303, Some(OperandSize::Dword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 153, 114, 148, 147, 7, 229, 26, 96, 84], OperandSize::Qword)
}

#[test]
fn vpsrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 114, 214, 108], OperandSize::Dword)
}

#[test]
fn vpsrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 869861259, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 114, 148, 128, 139, 7, 217, 51, 31], OperandSize::Dword)
}

#[test]
fn vpsrld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 85, 189, 114, 20, 64, 97], OperandSize::Dword)
}

#[test]
fn vpsrld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM14)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 101, 164, 114, 214, 79], OperandSize::Qword)
}

#[test]
fn vpsrld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 114, 19, 55], OperandSize::Qword)
}

#[test]
fn vpsrld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM17)), operand2: Some(IndirectDisplaced(RDX, 1855186161, Some(OperandSize::Dword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 179, 114, 146, 241, 228, 147, 110, 7], OperandSize::Qword)
}

#[test]
fn vpsrld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 114, 210, 70], OperandSize::Dword)
}

#[test]
fn vpsrld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 233931520, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 114, 148, 192, 0, 131, 241, 13, 17], OperandSize::Dword)
}

#[test]
fn vpsrld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 497442851, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 222, 114, 20, 117, 35, 96, 166, 29, 53], OperandSize::Dword)
}

#[test]
fn vpsrld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 45, 204, 114, 215, 110], OperandSize::Qword)
}

#[test]
fn vpsrld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM12)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 29, 203, 114, 16, 2], OperandSize::Qword)
}

#[test]
fn vpsrld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1939231232, Some(OperandSize::Dword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 37, 210, 114, 20, 149, 0, 82, 150, 115, 105], OperandSize::Qword)
}

#[test]
fn vpsrld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 210, 220], OperandSize::Dword)
}

#[test]
fn vpsrld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 1661523212, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 210, 159, 12, 213, 8, 99], OperandSize::Dword)
}

#[test]
fn vpsrld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 210, 232], OperandSize::Qword)
}

#[test]
fn vpsrld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 210, 44, 74], OperandSize::Qword)
}

#[test]
fn vpsrld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 210, 214], OperandSize::Dword)
}

#[test]
fn vpsrld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1486504311, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 210, 156, 254, 119, 65, 154, 88], OperandSize::Dword)
}

#[test]
fn vpsrld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 210, 232], OperandSize::Qword)
}

#[test]
fn vpsrld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1082979344, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 210, 28, 133, 16, 244, 140, 64], OperandSize::Qword)
}

#[test]
fn vpsrld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 210, 254], OperandSize::Dword)
}

#[test]
fn vpsrld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 1924481781, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 141, 210, 158, 245, 66, 181, 114], OperandSize::Dword)
}

#[test]
fn vpsrld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 143, 210, 214], OperandSize::Qword)
}

#[test]
fn vpsrld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 598347357, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 101, 139, 210, 28, 85, 93, 14, 170, 35], OperandSize::Qword)
}

#[test]
fn vpsrld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 210, 223], OperandSize::Dword)
}

#[test]
fn vpsrld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 755530943, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 210, 156, 254, 191, 124, 8, 45], OperandSize::Dword)
}

#[test]
fn vpsrld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 93, 172, 210, 233], OperandSize::Qword)
}

#[test]
fn vpsrld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 13, 166, 210, 2], OperandSize::Qword)
}

#[test]
fn vpsrld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 210, 194], OperandSize::Dword)
}

#[test]
fn vpsrld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 206, 210, 4, 72], OperandSize::Dword)
}

#[test]
fn vpsrld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 210, 198], OperandSize::Qword)
}

#[test]
fn vpsrld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1053524438, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 5, 205, 210, 4, 197, 214, 129, 203, 62], OperandSize::Qword)
}

