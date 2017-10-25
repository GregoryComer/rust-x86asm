use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 218, 238], OperandSize::Dword)
}

#[test]
fn vpminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 218, 28, 209], OperandSize::Dword)
}

#[test]
fn vpminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 218, 238], OperandSize::Qword)
}

#[test]
fn vpminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 553424467, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 218, 162, 83, 150, 252, 32], OperandSize::Qword)
}

#[test]
fn vpminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 218, 240], OperandSize::Dword)
}

#[test]
fn vpminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 142469585, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 218, 130, 209, 233, 125, 8], OperandSize::Dword)
}

#[test]
fn vpminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 218, 208], OperandSize::Qword)
}

#[test]
fn vpminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RBX, 1335927915, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 218, 163, 107, 164, 160, 79], OperandSize::Qword)
}

#[test]
fn vpminub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 218, 255], OperandSize::Dword)
}

#[test]
fn vpminub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 307947503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 218, 28, 77, 239, 231, 90, 18], OperandSize::Dword)
}

#[test]
fn vpminub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 45, 130, 218, 250], OperandSize::Qword)
}

#[test]
fn vpminub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 939778733, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 61, 139, 218, 36, 189, 173, 226, 3, 56], OperandSize::Qword)
}

#[test]
fn vpminub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 218, 243], OperandSize::Dword)
}

#[test]
fn vpminub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 758487246, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 218, 4, 133, 206, 152, 53, 45], OperandSize::Dword)
}

#[test]
fn vpminub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 117, 167, 218, 228], OperandSize::Qword)
}

#[test]
fn vpminub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RSI, 1199784925, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 53, 161, 218, 142, 221, 67, 131, 71], OperandSize::Qword)
}

#[test]
fn vpminub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 201, 218, 197], OperandSize::Dword)
}

#[test]
fn vpminub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 626313841, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 218, 20, 245, 113, 202, 84, 37], OperandSize::Dword)
}

#[test]
fn vpminub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 21, 196, 218, 217], OperandSize::Qword)
}

#[test]
fn vpminub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RBX, 630010869, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 29, 198, 218, 171, 245, 51, 141, 37], OperandSize::Qword)
}

