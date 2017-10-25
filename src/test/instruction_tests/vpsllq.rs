use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 115, 246, 17], OperandSize::Dword)
}

fn vpsllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 115, 245, 69], OperandSize::Qword)
}

fn vpsllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 115, 246, 26], OperandSize::Dword)
}

fn vpsllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 115, 244, 24], OperandSize::Qword)
}

fn vpsllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 243, 198], OperandSize::Dword)
}

fn vpsllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 993774136, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 243, 147, 56, 202, 59, 59], OperandSize::Dword)
}

fn vpsllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 243, 232], OperandSize::Qword)
}

fn vpsllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 56922335, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 243, 140, 193, 223, 144, 100, 3], OperandSize::Qword)
}

fn vpsllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 243, 255], OperandSize::Dword)
}

fn vpsllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 171969746, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 243, 190, 210, 12, 64, 10], OperandSize::Dword)
}

fn vpsllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 243, 235], OperandSize::Qword)
}

fn vpsllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RBX, 2082953296, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 243, 163, 80, 88, 39, 124], OperandSize::Qword)
}

fn vpsllq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 138, 243, 226], OperandSize::Dword)
}

fn vpsllq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 387159363, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 140, 243, 162, 67, 149, 19, 23], OperandSize::Dword)
}

fn vpsllq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 149, 139, 243, 254], OperandSize::Qword)
}

fn vpsllq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 141, 143, 243, 25], OperandSize::Qword)
}

fn vpsllq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 169, 243, 202], OperandSize::Dword)
}

fn vpsllq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 170, 243, 4, 120], OperandSize::Dword)
}

fn vpsllq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 205, 164, 243, 192], OperandSize::Qword)
}

fn vpsllq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 245, 163, 243, 20, 207], OperandSize::Qword)
}

fn vpsllq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 203, 243, 237], OperandSize::Dword)
}

fn vpsllq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 204, 243, 56], OperandSize::Dword)
}

fn vpsllq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 237, 193, 243, 196], OperandSize::Qword)
}

fn vpsllq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1508395381, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 206, 243, 52, 221, 117, 73, 232, 89], OperandSize::Qword)
}

