use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpackssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 107, 230], OperandSize::Dword)
}

fn vpackssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 107, 28, 254], OperandSize::Dword)
}

fn vpackssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 107, 229], OperandSize::Qword)
}

fn vpackssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 1318083452, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 107, 184, 124, 91, 144, 78], OperandSize::Qword)
}

fn vpackssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 107, 198], OperandSize::Dword)
}

fn vpackssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 107, 20, 138], OperandSize::Dword)
}

fn vpackssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 107, 209], OperandSize::Qword)
}

fn vpackssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 2119049728, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 107, 12, 69, 0, 34, 78, 126], OperandSize::Qword)
}

fn vpackssdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 107, 241], OperandSize::Dword)
}

fn vpackssdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 107, 40], OperandSize::Dword)
}

fn vpackssdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1304701249, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 158, 107, 52, 141, 65, 41, 196, 77], OperandSize::Dword)
}

fn vpackssdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 53, 134, 107, 193], OperandSize::Qword)
}

fn vpackssdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDX, 400669303, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 93, 139, 107, 178, 119, 186, 225, 23], OperandSize::Qword)
}

fn vpackssdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 653121635, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 77, 147, 107, 148, 83, 99, 216, 237, 38], OperandSize::Qword)
}

