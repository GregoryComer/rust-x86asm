use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn btr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 203], OperandSize::Word)
}

fn btr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 3, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 74, 3], OperandSize::Word)
}

fn btr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 218], OperandSize::Dword)
}

fn btr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 429128024, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 156, 127, 88, 249, 147, 25], OperandSize::Dword)
}

fn btr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 250], OperandSize::Qword)
}

fn btr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RCX, 40532178, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 137, 210, 120, 106, 2], OperandSize::Qword)
}

fn btr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 219], OperandSize::Word)
}

fn btr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(DI, 146, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 157, 146, 0], OperandSize::Word)
}

fn btr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 212], OperandSize::Dword)
}

fn btr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 19], OperandSize::Dword)
}

fn btr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 214], OperandSize::Qword)
}

fn btr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 28, 222], OperandSize::Qword)
}

fn btr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RBX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 203], OperandSize::Qword)
}

fn btr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RAX, Two, 609892381, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 60, 69, 29, 56, 90, 36], OperandSize::Qword)
}

fn btr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DI)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 247, 10], OperandSize::Word)
}

fn btr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 48, 118], OperandSize::Word)
}

fn btr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(BX)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 243, 123], OperandSize::Dword)
}

fn btr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 52, 187, 106], OperandSize::Dword)
}

fn btr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DI)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 247, 79], OperandSize::Qword)
}

fn btr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 50, 113], OperandSize::Qword)
}

fn btr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 241, 55], OperandSize::Word)
}

fn btr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(BX, 169, Some(OperandSize::Dword), None)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 183, 169, 0, 127], OperandSize::Word)
}

fn btr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 243, 31], OperandSize::Dword)
}

fn btr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1369551052, Some(OperandSize::Dword), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 133, 204, 176, 161, 81, 66], OperandSize::Dword)
}

fn btr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 245, 19], OperandSize::Qword)
}

fn btr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1847951329, Some(OperandSize::Dword), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 181, 225, 127, 37, 110, 82], OperandSize::Qword)
}

fn btr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 243, 98], OperandSize::Qword)
}

fn btr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 52, 123, 25], OperandSize::Qword)
}

