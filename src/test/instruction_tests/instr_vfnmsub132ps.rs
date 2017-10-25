use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 158, 194], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 56541359, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 158, 156, 115, 175, 192, 94, 3], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 158, 201], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 605455733, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 158, 180, 193, 117, 133, 22, 36], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 158, 250], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 158, 36, 251], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 158, 213], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RBX, 941785360, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 158, 155, 16, 129, 34, 56], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 158, 253], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1497921559, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 158, 172, 152, 23, 120, 72, 89], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 153, 158, 36, 179], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 117, 133, 158, 201], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 2019375291, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 85, 132, 158, 52, 245, 187, 56, 93, 120], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 45, 151, 158, 44, 70], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 158, 247], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 2091047382, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 158, 146, 214, 217, 162, 124], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1479893894, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 190, 158, 60, 205, 134, 99, 53, 88], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 45, 162, 158, 238], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 620769578, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 77, 174, 158, 188, 251, 42, 49, 0, 37], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 101, 185, 158, 12, 80], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 218, 158, 222], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 26219068, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 202, 158, 4, 125, 60, 18, 144, 1], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 222, 158, 52, 185], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 190, 158, 231], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1152241641, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 109, 207, 158, 140, 134, 233, 207, 173, 68], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 45, 217, 158, 35], OperandSize::Qword)
}

