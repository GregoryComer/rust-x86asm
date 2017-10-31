use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 150, 210], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 24176167, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 150, 180, 192, 39, 230, 112, 1], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 150, 254], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 150, 18], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 150, 211], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 737359843, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 150, 28, 149, 227, 55, 243, 43], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 150, 240], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RCX, 856515412, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 150, 177, 84, 99, 13, 51], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 150, 254], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 150, 2], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 150, 17], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 77, 138, 150, 254], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 137, 150, 52, 184], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 146, 150, 52, 139], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 169, 150, 249], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 333812545, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 174, 150, 138, 65, 147, 229, 19], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 835167327, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 185, 150, 144, 95, 164, 199, 49], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 101, 172, 150, 240], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 117, 162, 150, 40], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 53, 179, 150, 28, 223], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 222, 150, 232], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 205, 150, 20, 95], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1317458235, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 222, 150, 36, 253, 59, 209, 134, 78], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 77, 245, 150, 249], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1486073219, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 85, 206, 150, 172, 207, 131, 173, 147, 88], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1447734815, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 61, 212, 150, 156, 203, 31, 174, 74, 86], OperandSize::Qword)
}

