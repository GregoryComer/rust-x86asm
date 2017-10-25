use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 18, 36, 118], OperandSize::Dword)
}

fn vmovlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 18, 18], OperandSize::Qword)
}

fn vmovlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 18, 12, 241], OperandSize::Dword)
}

fn vmovlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RAX, 212064698, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 12, 0, 18, 144, 186, 217, 163, 12], OperandSize::Qword)
}

fn vmovlps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 613325815, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 132, 207, 247, 155, 142, 36], OperandSize::Dword)
}

fn vmovlps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectDisplaced(RAX, 788738533, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 176, 229, 49, 3, 47], OperandSize::Qword)
}

fn vmovlps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1655781405, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 164, 131, 29, 56, 177, 98], OperandSize::Dword)
}

fn vmovlps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 19, 20, 128], OperandSize::Qword)
}

