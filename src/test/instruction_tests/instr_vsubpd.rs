use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 92, 193], OperandSize::Dword)
}

#[test]
fn vsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1675709117, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 92, 139, 189, 74, 225, 99], OperandSize::Dword)
}

#[test]
fn vsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 92, 209], OperandSize::Qword)
}

#[test]
fn vsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDI, 1052929673, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 92, 191, 137, 110, 194, 62], OperandSize::Qword)
}

#[test]
fn vsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 92, 233], OperandSize::Dword)
}

#[test]
fn vsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 92, 26], OperandSize::Dword)
}

#[test]
fn vsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 92, 199], OperandSize::Qword)
}

#[test]
fn vsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 92, 35], OperandSize::Qword)
}

#[test]
fn vsubpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 92, 224], OperandSize::Dword)
}

#[test]
fn vsubpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 137, 92, 55], OperandSize::Dword)
}

#[test]
fn vsubpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 154, 92, 52, 114], OperandSize::Dword)
}

#[test]
fn vsubpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 165, 135, 92, 237], OperandSize::Qword)
}

#[test]
fn vsubpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RDI, 1726066078, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 173, 141, 92, 143, 158, 173, 225, 102], OperandSize::Qword)
}

#[test]
fn vsubpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 879814623, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 157, 148, 92, 132, 91, 223, 231, 112, 52], OperandSize::Qword)
}

#[test]
fn vsubpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 170, 92, 248], OperandSize::Dword)
}

#[test]
fn vsubpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 909866173, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 169, 92, 172, 248, 189, 116, 59, 54], OperandSize::Dword)
}

#[test]
fn vsubpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1469483337, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 185, 92, 156, 202, 73, 137, 150, 87], OperandSize::Dword)
}

#[test]
fn vsubpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 133, 174, 92, 206], OperandSize::Qword)
}

#[test]
fn vsubpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 181, 169, 92, 52, 129], OperandSize::Qword)
}

#[test]
fn vsubpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 2109473869, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 189, 92, 60, 117, 77, 4, 188, 125], OperandSize::Qword)
}

#[test]
fn vsubpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 254, 92, 226], OperandSize::Dword)
}

#[test]
fn vsubpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 92, 28, 210], OperandSize::Dword)
}

#[test]
fn vsubpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 218, 92, 59], OperandSize::Dword)
}

#[test]
fn vsubpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 197, 157, 92, 213], OperandSize::Qword)
}

#[test]
fn vsubpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 2074118347, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 149, 202, 92, 188, 241, 203, 136, 160, 123], OperandSize::Qword)
}

#[test]
fn vsubpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RAX, 1102214571, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 165, 222, 92, 144, 171, 117, 178, 65], OperandSize::Qword)
}

