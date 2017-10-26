use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 152, 231], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 308830600, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 152, 12, 157, 136, 97, 104, 18], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 152, 230], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 152, 36, 90], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 152, 226], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 152, 42], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 152, 199], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1116322542, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 152, 180, 194, 238, 186, 137, 66], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 152, 228], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 190309972, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 152, 60, 157, 84, 230, 87, 11], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 157, 152, 32], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 85, 135, 152, 207], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 13, 133, 152, 47], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 1045016237, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 93, 153, 152, 188, 145, 173, 174, 73, 62], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 152, 230], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 152, 28, 152], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 189, 152, 56], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 85, 173, 152, 207], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 109, 169, 152, 35], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1330304904, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 45, 188, 152, 172, 134, 136, 215, 74, 79], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 191, 152, 230], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 788714996, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 201, 152, 28, 149, 244, 213, 2, 47], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 218, 152, 14], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 77, 212, 152, 237], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 2039577379, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 5, 201, 152, 172, 240, 35, 123, 145, 121], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 77, 213, 152, 16], OperandSize::Qword)
}

