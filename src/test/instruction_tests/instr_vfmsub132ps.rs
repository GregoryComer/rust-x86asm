use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 154, 250], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 143557351, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 154, 138, 231, 130, 142, 8], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 154, 205], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 154, 58], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 154, 204], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1764473425, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 154, 188, 218, 81, 186, 43, 105], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 154, 223], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 68702644, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 154, 171, 180, 81, 24, 4], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 142, 154, 202], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1949905720, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 154, 20, 149, 56, 51, 57, 116], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 154, 44, 137], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 21, 133, 154, 253], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 101, 143, 154, 11], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1231227241, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 85, 155, 154, 20, 221, 105, 9, 99, 73], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 154, 238], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1348106799, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 154, 169, 47, 122, 90, 80], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 2046829047, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 186, 154, 164, 144, 247, 33, 0, 122], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 29, 171, 154, 223], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RBX, 645459737, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 109, 172, 154, 187, 25, 239, 120, 38], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RBX, 62953452, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 45, 182, 154, 155, 236, 151, 192, 3], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 217, 154, 196], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 207, 154, 4, 127], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 220, 154, 12, 247], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 21, 149, 154, 195], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 2034581347, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 77, 198, 154, 4, 245, 99, 63, 69, 121], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 109, 212, 154, 35], OperandSize::Qword)
}

