use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtph2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 222], OperandSize::Dword)
}

fn vcvtph2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 2135390151, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 156, 138, 199, 119, 71, 127], OperandSize::Dword)
}

fn vcvtph2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 239], OperandSize::Qword)
}

fn vcvtph2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 7], OperandSize::Qword)
}

fn vcvtph2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 205], OperandSize::Dword)
}

fn vcvtph2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ESI, 1674930285, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 158, 109, 104, 213, 99], OperandSize::Dword)
}

fn vcvtph2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 206], OperandSize::Qword)
}

fn vcvtph2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 122211653, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 132, 87, 69, 205, 72, 7], OperandSize::Qword)
}

fn vcvtph2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 19, 250], OperandSize::Dword)
}

fn vcvtph2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 1486500457, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 19, 162, 105, 50, 154, 88], OperandSize::Dword)
}

fn vcvtph2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 125, 139, 19, 254], OperandSize::Qword)
}

fn vcvtph2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1770900572, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 138, 19, 28, 85, 92, 204, 141, 105], OperandSize::Qword)
}

fn vcvtph2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 19, 219], OperandSize::Dword)
}

fn vcvtph2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EBX, 1533731597, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 19, 171, 13, 227, 106, 91], OperandSize::Dword)
}

fn vcvtph2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 125, 174, 19, 222], OperandSize::Qword)
}

fn vcvtph2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM30)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 19, 49], OperandSize::Qword)
}

fn vcvtph2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 154, 19, 240], OperandSize::Dword)
}

fn vcvtph2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 870147783, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 19, 180, 147, 199, 102, 221, 51], OperandSize::Dword)
}

fn vcvtph2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 157, 19, 255], OperandSize::Qword)
}

fn vcvtph2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 2117141025, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 19, 60, 149, 33, 2, 49, 126], OperandSize::Qword)
}

