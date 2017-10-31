use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 223, 199], OperandSize::Dword)
}

#[test]
fn vpandnd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 137, 223, 6], OperandSize::Dword)
}

#[test]
fn vpandnd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 155, 223, 7], OperandSize::Dword)
}

#[test]
fn vpandnd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 13, 129, 223, 203], OperandSize::Qword)
}

#[test]
fn vpandnd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 45, 133, 223, 38], OperandSize::Qword)
}

#[test]
fn vpandnd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 745937685, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 93, 150, 223, 156, 199, 21, 27, 118, 44], OperandSize::Qword)
}

#[test]
fn vpandnd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 223, 197], OperandSize::Dword)
}

#[test]
fn vpandnd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 1804301653, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 223, 150, 85, 117, 139, 107], OperandSize::Dword)
}

#[test]
fn vpandnd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 995531515, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 190, 223, 180, 158, 251, 154, 86, 59], OperandSize::Dword)
}

#[test]
fn vpandnd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 85, 165, 223, 243], OperandSize::Qword)
}

#[test]
fn vpandnd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1191942156, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 53, 175, 223, 180, 143, 12, 152, 11, 71], OperandSize::Qword)
}

#[test]
fn vpandnd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 462376679, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 61, 180, 223, 44, 245, 231, 78, 143, 27], OperandSize::Qword)
}

#[test]
fn vpandnd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 201, 223, 228], OperandSize::Dword)
}

#[test]
fn vpandnd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 223, 33], OperandSize::Dword)
}

#[test]
fn vpandnd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 117, 217, 223, 55], OperandSize::Dword)
}

#[test]
fn vpandnd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 117, 197, 223, 210], OperandSize::Qword)
}

#[test]
fn vpandnd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 838373213, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 61, 204, 223, 188, 127, 93, 143, 248, 49], OperandSize::Qword)
}

#[test]
fn vpandnd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RBX, 2058617226, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 222, 223, 179, 138, 1, 180, 122], OperandSize::Qword)
}

