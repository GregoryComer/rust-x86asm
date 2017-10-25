use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 22, 239], OperandSize::Dword)
}

#[test]
fn vpermpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 471822094, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 22, 164, 80, 14, 111, 31, 28], OperandSize::Dword)
}

#[test]
fn vpermpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 191, 22, 36, 119], OperandSize::Dword)
}

#[test]
fn vpermpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 173, 161, 22, 243], OperandSize::Qword)
}

#[test]
fn vpermpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 623982274, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 181, 165, 22, 60, 181, 194, 54, 49, 37], OperandSize::Qword)
}

#[test]
fn vpermpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1573128211, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 133, 190, 22, 156, 176, 19, 8, 196, 93], OperandSize::Qword)
}

#[test]
fn vpermpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 22, 237], OperandSize::Dword)
}

#[test]
fn vpermpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 159291615, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 204, 22, 180, 130, 223, 152, 126, 9], OperandSize::Dword)
}

#[test]
fn vpermpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 492034055, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 22, 180, 192, 7, 216, 83, 29], OperandSize::Dword)
}

#[test]
fn vpermpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 193, 22, 208], OperandSize::Qword)
}

#[test]
fn vpermpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1907732912, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 213, 204, 22, 4, 93, 176, 177, 181, 113], OperandSize::Qword)
}

#[test]
fn vpermpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 157, 222, 22, 26], OperandSize::Qword)
}

#[test]
fn vpermpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 231, 83], OperandSize::Dword)
}

#[test]
fn vpermpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EBX, 1333821926, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 139, 230, 129, 128, 79, 103], OperandSize::Dword)
}

#[test]
fn vpermpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 213, 23], OperandSize::Qword)
}

#[test]
fn vpermpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1137376484, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 156, 71, 228, 252, 202, 67, 61], OperandSize::Qword)
}

#[test]
fn vpermpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 1, 209, 122], OperandSize::Dword)
}

#[test]
fn vpermpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1187186504, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 1, 180, 199, 72, 7, 195, 70, 23], OperandSize::Dword)
}

#[test]
fn vpermpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ESI, 2093954891, Some(OperandSize::Qword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 189, 1, 134, 75, 55, 207, 124, 0], OperandSize::Dword)
}

#[test]
fn vpermpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 253, 173, 1, 252, 127], OperandSize::Qword)
}

#[test]
fn vpermpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 250133042, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 253, 175, 1, 20, 149, 50, 186, 232, 14, 109], OperandSize::Qword)
}

#[test]
fn vpermpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM19)), operand2: Some(IndirectDisplaced(RBX, 182689314, Some(OperandSize::Qword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 186, 1, 155, 34, 158, 227, 10, 121], OperandSize::Qword)
}

#[test]
fn vpermpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 1, 216, 54], OperandSize::Dword)
}

#[test]
fn vpermpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1509070367, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 1, 20, 125, 31, 150, 242, 89, 120], OperandSize::Dword)
}

#[test]
fn vpermpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ECX, 2032327555, Some(OperandSize::Qword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 221, 1, 153, 131, 219, 34, 121, 16], OperandSize::Dword)
}

#[test]
fn vpermpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 19, 253, 204, 1, 242, 49], OperandSize::Qword)
}

#[test]
fn vpermpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1842995869, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 253, 202, 1, 164, 80, 157, 226, 217, 109, 115], OperandSize::Qword)
}

#[test]
fn vpermpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1710933192, Some(OperandSize::Qword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 219, 1, 44, 125, 200, 196, 250, 101, 67], OperandSize::Qword)
}

