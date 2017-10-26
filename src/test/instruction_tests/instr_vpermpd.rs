use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 22, 208], OperandSize::Dword)
}

#[test]
fn vpermpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 22, 42], OperandSize::Dword)
}

#[test]
fn vpermpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 1027317142, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 187, 22, 171, 150, 157, 59, 61], OperandSize::Dword)
}

#[test]
fn vpermpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 237, 173, 22, 240], OperandSize::Qword)
}

#[test]
fn vpermpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RDX, 676201015, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 181, 164, 22, 146, 55, 2, 78, 40], OperandSize::Qword)
}

#[test]
fn vpermpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 358300174, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 237, 180, 22, 148, 139, 14, 58, 91, 21], OperandSize::Qword)
}

#[test]
fn vpermpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 207, 22, 252], OperandSize::Dword)
}

#[test]
fn vpermpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 202, 22, 28, 151], OperandSize::Dword)
}

#[test]
fn vpermpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 217, 22, 4, 78], OperandSize::Dword)
}

#[test]
fn vpermpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 229, 202, 22, 233], OperandSize::Qword)
}

#[test]
fn vpermpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1207771375, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 201, 22, 44, 245, 239, 32, 253, 71], OperandSize::Qword)
}

#[test]
fn vpermpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 189, 220, 22, 48], OperandSize::Qword)
}

#[test]
fn vpermpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 240, 66], OperandSize::Dword)
}

#[test]
fn vpermpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDI, 212376894, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 175, 62, 157, 168, 12, 124], OperandSize::Dword)
}

#[test]
fn vpermpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 196, 34], OperandSize::Qword)
}

#[test]
fn vpermpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 41, 28], OperandSize::Qword)
}

#[test]
fn vpermpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 1, 197, 44], OperandSize::Dword)
}

#[test]
fn vpermpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDX, 1633990142, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 1, 186, 254, 181, 100, 97, 26], OperandSize::Dword)
}

#[test]
fn vpermpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDI, 47405421, Some(OperandSize::Qword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 189, 1, 183, 109, 89, 211, 2, 119], OperandSize::Dword)
}

#[test]
fn vpermpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 253, 169, 1, 242, 115], OperandSize::Qword)
}

#[test]
fn vpermpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1569655900, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 253, 175, 1, 172, 210, 92, 12, 143, 93, 50], OperandSize::Qword)
}

#[test]
fn vpermpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM31)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 186, 1, 59, 80], OperandSize::Qword)
}

#[test]
fn vpermpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 1, 210, 35], OperandSize::Dword)
}

#[test]
fn vpermpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDI, 2078650901, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 1, 191, 21, 178, 229, 123, 30], OperandSize::Dword)
}

#[test]
fn vpermpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 219, 1, 56, 5], OperandSize::Dword)
}

#[test]
fn vpermpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM16)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 253, 205, 1, 192, 108], OperandSize::Qword)
}

#[test]
fn vpermpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectDisplaced(RDI, 2085986865, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 253, 206, 1, 135, 49, 162, 85, 124, 15], OperandSize::Qword)
}

#[test]
fn vpermpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 897531003, Some(OperandSize::Qword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 253, 218, 1, 164, 246, 123, 60, 127, 53, 100], OperandSize::Qword)
}

