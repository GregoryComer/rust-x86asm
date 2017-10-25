use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 115, 213, 88], OperandSize::Dword)
}

fn vpsrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 115, 211, 48], OperandSize::Qword)
}

fn vpsrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 115, 209, 120], OperandSize::Dword)
}

fn vpsrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 211, 119], OperandSize::Qword)
}

fn vpsrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 211, 218], OperandSize::Dword)
}

fn vpsrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 211, 36, 250], OperandSize::Dword)
}

fn vpsrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 211, 212], OperandSize::Qword)
}

fn vpsrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 211, 33], OperandSize::Qword)
}

fn vpsrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 211, 210], OperandSize::Dword)
}

fn vpsrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1036614190, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 211, 140, 135, 46, 122, 201, 61], OperandSize::Dword)
}

fn vpsrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 211, 233], OperandSize::Qword)
}

fn vpsrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 211, 36, 139], OperandSize::Qword)
}

fn vpsrlq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 142, 211, 247], OperandSize::Dword)
}

fn vpsrlq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 211, 42], OperandSize::Dword)
}

fn vpsrlq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 189, 141, 211, 206], OperandSize::Qword)
}

fn vpsrlq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 189, 143, 211, 56], OperandSize::Qword)
}

fn vpsrlq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 211, 198], OperandSize::Dword)
}

fn vpsrlq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 687841335, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 172, 211, 169, 55, 160, 255, 40], OperandSize::Dword)
}

fn vpsrlq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 157, 162, 211, 232], OperandSize::Qword)
}

fn vpsrlq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RAX, 1411832117, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 165, 172, 211, 176, 53, 217, 38, 84], OperandSize::Qword)
}

fn vpsrlq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 211, 232], OperandSize::Dword)
}

fn vpsrlq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 86367833, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 211, 156, 70, 89, 222, 37, 5], OperandSize::Dword)
}

fn vpsrlq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 157, 193, 211, 204], OperandSize::Qword)
}

fn vpsrlq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 204, 211, 4, 134], OperandSize::Qword)
}

