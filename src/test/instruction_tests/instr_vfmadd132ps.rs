use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 152, 222], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 2023728348, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 152, 137, 220, 164, 159, 120], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 152, 206], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 152, 44, 120], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 152, 203], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 1419777870, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 152, 152, 78, 23, 160, 84], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 152, 221], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RSI, 1099914830, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 152, 158, 78, 94, 143, 65], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 152, 248], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 379903291, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 152, 151, 59, 221, 164, 22], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 157, 152, 12, 126], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 117, 139, 152, 252], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1244187030, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 37, 138, 152, 20, 253, 150, 201, 40, 74], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 77, 147, 152, 19], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 152, 223], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 152, 52, 200], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 800397539, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 187, 152, 137, 227, 24, 181, 47], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 163, 152, 246], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 152, 32], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 455135113, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 109, 181, 152, 156, 74, 137, 207, 32, 27], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 253, 152, 203], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 152, 52, 113], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 222, 152, 44, 153], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 125, 186, 152, 218], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1469482782, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 45, 195, 152, 4, 117, 30, 135, 150, 87], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1433558230, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 61, 218, 152, 4, 181, 214, 92, 114, 85], OperandSize::Qword)
}

